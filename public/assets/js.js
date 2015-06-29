(function(){
    var app = angular.module('ScheduleGenApp', ['ngMaterial', 'ngMdIcons']);

    app.controller('AppCtrl', function ($scope) {
        // TEAMS
        $scope.teams = [ 
            { name: "Team 1", id: 0 },
            { name: "Team 2", id: 1 }];

        $scope.removeTeam = function(id) {
            var oldTeams = $scope.teams;
            $scope.teams = [];
            angular.forEach(oldTeams, function(team) {
                if(team.id != id) {
                    $scope.teams.push(team);
                }
            });
        };

        $scope.addTeam = function() { 
            var newId = 0;

            if ($scope.teams.length != 0) {
                newId = $scope.teams[$scope.teams.length - 1].id + 1;
            }

            $scope.teams.push({ 
                name: '',
                id: newId
            });
        };


        // LOCATIONS
        $scope.locations = [ 
            { name: "Baseball Field 1", id: 0 },
            { name: "Baseball Field 2", id: 1 }];

        $scope.removeLocation = function(id) {
            var oldLocations = $scope.locations;
            $scope.locations = [];
            angular.forEach(oldLocations, function(location) {
                if(location.id != id) {
                    $scope.locations.push(location);
                }
            });
        };

        $scope.addLocation = function() { 
            var newId = 0;

            if ($scope.locations.length != 0) {
                newId = $scope.locations[$scope.locations.length - 1].id + 1;
            }

            $scope.locations.push({ 
                name: '',
                id: newId
            });
        };

        // START AND END DATES
        $scope.startDate = null;
        $scope.endDate = null;

        // TIMES
        $scope.times= [ 
            { time: '', id: 0 , locationOptions: $scope.locations, locations: [] },
            { time: '', id: 1 , locationOptions: $scope.locations, locations: [] }];

        $scope.removeTime = function(id) {
            var oldTimes= $scope.times;
            $scope.times = [];
            angular.forEach(oldTimes, function(time) {
                if(time.id != id) {
                    $scope.times.push(time);
                }
            });
        };

        $scope.addTime = function() { 
            var newId = 0;

            if ($scope.times.length != 0) {
                newId = $scope.times[$scope.times.length - 1].id + 1;
            }

            $scope.times.push({ 
                time: '',
                id: newId,
                locationOptions: $scope.locations,
                locations: []
            });
        };
    });
})();
